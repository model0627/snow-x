"""
Database connection and session management
"""
from contextlib import contextmanager
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session
from sqlalchemy.ext.declarative import declarative_base
from .config import settings

# Create engine using settings
engine = create_engine(settings.DATABASE_URL, echo=False)

# Create sessionmaker
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

# Base class for models
Base = declarative_base()


def get_db_session() -> Session:
    """Get a database session"""
    return SessionLocal()


@contextmanager
def get_db_session_context():
    """Context manager for database sessions"""
    db = SessionLocal()
    try:
        yield db
        db.commit()
    except Exception:
        db.rollback()
        raise
    finally:
        db.close()