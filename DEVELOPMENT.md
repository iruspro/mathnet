# MathNet
## Setup development environment
### Pre-commit hooks:
```bash
cp pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```
### Start database
```bash
docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:17
```
