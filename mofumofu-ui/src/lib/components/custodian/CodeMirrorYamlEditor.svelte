<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { EditorView, keymap, lineNumbers, highlightActiveLineGutter, highlightSpecialChars, drawSelection, dropCursor, rectangularSelection, crosshairCursor, highlightActiveLine, placeholder as placeholderExtension } from '@codemirror/view';
	import { EditorState, Compartment } from '@codemirror/state';
	import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
	import { yaml } from '@codemirror/lang-yaml';
	import { syntaxHighlighting, defaultHighlightStyle, bracketMatching, foldGutter, indentOnInput } from '@codemirror/language';

	let {
		value = $bindable(''),
		readOnly = false,
		placeholder = 'Enter YAML content...',
		onValidationError
	}: {
		value?: string;
		readOnly?: boolean;
		placeholder?: string;
		onValidationError?: (error: string | null) => void;
	} = $props();

	let editorContainer: HTMLDivElement;
	let editorView: EditorView | null = null;
	let validationError = $state<string | null>(null);

	const readOnlyCompartment = new Compartment();

	onMount(() => {
		if (!editorContainer) {
			console.error('Editor container not found');
			return;
		}

		console.log('Mounting CodeMirror editor with value:', value);
		console.log('Editor container:', editorContainer);
		console.log('ReadOnly:', readOnly);

		const extensions = [
			lineNumbers(),
			highlightActiveLineGutter(),
			highlightSpecialChars(),
			history(),
			foldGutter(),
			drawSelection(),
			dropCursor(),
			EditorState.allowMultipleSelections.of(true),
			indentOnInput(),
			syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
			bracketMatching(),
			rectangularSelection(),
			crosshairCursor(),
			highlightActiveLine(),
			keymap.of([
				...defaultKeymap,
				...historyKeymap,
				indentWithTab // Tab 키로 들여쓰기
			]),
			yaml(),
			readOnlyCompartment.of(EditorState.readOnly.of(readOnly)),
			EditorView.updateListener.of((update) => {
				if (update.docChanged) {
					const newValue = update.state.doc.toString();
					console.log('Document changed:', newValue);
					value = newValue;
					validateYaml(newValue);
				}
			}),
			EditorView.theme({
				'&': {
					height: '100%',
					fontSize: '14px',
					fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace',
					backgroundColor: '#ffffff',
					color: '#1f2937' // gray-800 - 기본 텍스트 색상
				},
				'&.cm-focused': {
					outline: '2px solid #f97316',
					outlineOffset: '-2px'
				},
				'.cm-scroller': {
					overflow: 'auto',
					fontFamily: 'inherit'
				},
				'.cm-content': {
					padding: '10px 0',
					minHeight: '400px',
					caretColor: '#f97316',
					color: '#1f2937' // gray-800
				},
				'.cm-line': {
					padding: '0 4px',
					color: '#1f2937' // gray-800
				},
				'.cm-cursor': {
					borderLeftColor: '#f97316',
					borderLeftWidth: '2px'
				},
				'&.cm-focused .cm-cursor': {
					borderLeftColor: '#f97316'
				},
				'&.cm-focused .cm-selectionBackground, ::selection': {
					backgroundColor: '#fed7aa' // orange-200
				},
				'.cm-activeLine': {
					backgroundColor: '#fff7ed' // orange-50
				},
				'.cm-activeLineGutter': {
					backgroundColor: '#ffedd5' // orange-100
				},
				'.cm-gutters': {
					backgroundColor: '#f9fafb', // gray-50
					color: '#6b7280', // gray-500
					border: 'none'
				},
				// YAML syntax highlighting
				'.cm-content .ͼe': { // property
					color: '#0369a1' // sky-700
				},
				'.cm-content .ͼd': { // string
					color: '#15803d' // green-700
				},
				'.cm-content .ͼc': { // number
					color: '#c026d3' // fuchsia-600
				},
				'.cm-content .ͼb': { // bool
					color: '#dc2626' // red-600
				},
				'.cm-content .ͼa': { // null
					color: '#9333ea' // purple-600
				}
			})
		];

		// Add placeholder if provided
		if (placeholder && !value) {
			extensions.push(placeholderExtension(placeholder));
		}

		const startState = EditorState.create({
			doc: value || '',
			extensions: extensions
		});

		console.log('Creating EditorView...');
		editorView = new EditorView({
			state: startState,
			parent: editorContainer
		});

		console.log('CodeMirror editor mounted successfully');
		console.log('EditorView:', editorView);
		console.log('Editor state:', editorView.state);

		// Initial validation
		validateYaml(value || '');

		// Focus the editor if not readonly
		if (!readOnly) {
			setTimeout(() => {
				console.log('Attempting to focus editor...');
				editorView?.focus();
				console.log('Editor focused');
			}, 100);
		}
	});

	onDestroy(() => {
		if (editorView) {
			editorView.destroy();
		}
	});

	// Update editor when readOnly prop changes
	$effect(() => {
		if (editorView && readOnly !== undefined) {
			editorView.dispatch({
				effects: readOnlyCompartment.reconfigure(EditorState.readOnly.of(readOnly))
			});
		}
	});

	// Update editor when value changes externally
	$effect(() => {
		if (editorView && value !== editorView.state.doc.toString()) {
			editorView.dispatch({
				changes: {
					from: 0,
					to: editorView.state.doc.length,
					insert: value
				}
			});
		}
	});

	function validateYaml(content: string) {
		if (!content.trim()) {
			validationError = null;
			if (onValidationError) onValidationError(null);
			return;
		}

		try {
			// Basic YAML structure validation
			const lines = content.split('\n');
			let indentStack: number[] = [];

			for (let i = 0; i < lines.length; i++) {
				const line = lines[i];

				// Skip empty lines and comments
				if (!line.trim() || line.trim().startsWith('#')) continue;

				// Check indentation (must be 2 spaces)
				const leadingSpaces = line.match(/^ */)?.[0].length || 0;
				if (leadingSpaces % 2 !== 0) {
					validationError = `Line ${i + 1}: Indentation must be multiples of 2 spaces`;
					if (onValidationError) onValidationError(validationError);
					return;
				}

				// Check for tabs
				if (line.includes('\t')) {
					validationError = `Line ${i + 1}: Tabs are not allowed, use spaces for indentation`;
					if (onValidationError) onValidationError(validationError);
					return;
				}
			}

			// Check for required 'policies:' key for Cloud Custodian
			if (!content.includes('policies:')) {
				validationError = 'YAML must contain a "policies:" section for Cloud Custodian';
				if (onValidationError) onValidationError(validationError);
				return;
			}

			validationError = null;
			if (onValidationError) onValidationError(null);
		} catch (err) {
			const error = err instanceof Error ? err.message : 'Invalid YAML syntax';
			validationError = error;
			if (onValidationError) onValidationError(error);
		}
	}

	// Public methods
	export function getValue(): string {
		return editorView ? editorView.state.doc.toString() : value;
	}

	export function setValue(newValue: string) {
		if (editorView) {
			editorView.dispatch({
				changes: {
					from: 0,
					to: editorView.state.doc.length,
					insert: newValue
				}
			});
		}
		value = newValue;
	}

	export function focus() {
		if (editorView) {
			editorView.focus();
		}
	}
</script>

<div class="yaml-editor-wrapper h-full flex flex-col">
	<div bind:this={editorContainer} class="flex-1 overflow-hidden rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900"></div>

	{#if validationError}
		<div class="mt-2 rounded-md bg-red-50 p-3 dark:bg-red-950/30">
			<p class="text-sm text-red-600 dark:text-red-400">
				⚠️ {validationError}
			</p>
		</div>
	{/if}
</div>

<style>
	:global(.yaml-editor-wrapper .cm-editor) {
		height: 100%;
	}

	:global(.yaml-editor-wrapper .cm-scroller) {
		overflow: auto;
	}
</style>
