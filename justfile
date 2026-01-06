
# Runs app with MyField Config included
mf:
	CONFIG_PATH="static/my_field.toml" uv run uvicorn gamegoly.main:app --reload

test:
	uv run -m unittest discover -s tests
