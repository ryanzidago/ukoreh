# ukoreh

## maintenance-mode
Activates Heroku's maintenance mode for all apps listed in `src/configs/heroku_apps.yml`, given a maintenance window. 
The `src/configs/heroku_apps.yml` must present the following structure:
```yml 
apps: 
    <name of my app>:
        - <name of the environment>
```
### Example

For example, given the apps `my-app-1` that needs to be put on maintenance mode for the environments `staging` and `production`, and another app `my-app-2` that must only be put on maintenance mode for staging:
```yml
apps:
    my-app-1:
        - staging
        - production
    my-app-2:
        -staging
```

Then, `my-app-1` (staging and production) and `my-app-2` (staging only) can be put on maintenance mode from `2022-02-19T14:00:00` UTC to `2022-02-19T14:30:00` UTC like so:
```
ukoreh maintenance-mode --start 2022-02-19T14:00:00 --end 2022-02-19T14:30:00
```