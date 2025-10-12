import argparse

from . import gameconfig

def main(**kwargs):
    config_file_path = kwargs["config_path"]
    gameconfig.create_config(config_file_path)


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument('--config_path')
    args = p.parse_args()
    main(**vars(args))
