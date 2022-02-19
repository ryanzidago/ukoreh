# ukoreh

## maintenance-mode
Activates Heroku's maintenance mode for all apps listed in `src/configs/heroku_apps.yml`, given a maintenance window. 
The `src/configs/heroku_apps.yml` must present the following structure:
```yml 
apps: 
    <name of my app>:
        - <name of the environment>
```

For example, given the apps `my-app-1` that needs to be put on maintenance mode for the environments `staging` and `production`, and another app `my-app-2` that must only be put on maintenance mode for staging:
```yml
apps:
    my-app-1:
        - staging
        - production
    my-app-2:
        -staging
```