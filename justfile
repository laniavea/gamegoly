
# Runs app with MyField Config included
mf:
	uv run -m src.gamegoly.main --config_path="static/my_field.toml"

test:
	uv run -m unittest discover -s tests
