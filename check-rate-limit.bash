#!/usr/bin/env bash

for i in {1..200}; do
    curl --request GET \
         --url 'http://localhost:8000/get_account?username=raju' \
         --header 'Authorization: Bearer 7f853b13-fc18-4cbb-a80e-3c6002ef7bb4' \
         --header 'User-Agent: insomnia/8.6.1' \
         --cookie csrf_token=a%2F3qCn6cG%2FTqavdeUGBJS2L6SBxeq39KMxLYF7z1qJoggaPOr0lWtJQ6urARVhUJlMFikIyVsIvw2V5TB%2FWNbg%3D%3D && echo
done
