# Standard Library
import logging
from pathlib import Path


def get_logger(name: str) -> logging.Logger:
    """Return a logger with the given name."""
    logger = logging.getLogger(name)
    logger.setLevel(logging.CRITICAL)
    return logger


def edit_logger_for_verbosity(
    logger: logging.Logger, verbose: bool, full_quiet: bool
) -> None:
    """Edit the logger's level based on verbosity.

    Args:
        logger (logging.Logger): The logger to edit.
        verbose (bool): Whether the user wants verbose output.
        full_quiet (bool): Whether the user wants no output.
    """
    if verbose:
        handle = logging.FileHandler(f"{Path.cwd()}/{logger.name}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        logger.addHandler(handle)
        logger.setLevel(logging.DEBUG)
    if full_quiet:
        logging.disable(logging.CRITICAL)
