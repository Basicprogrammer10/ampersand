- https://docs.github.com/en/free-pro-team@latest/rest/deployments/deployments?apiVersion=2022-11-28#create-a-deployment
- https://docs.github.com/en/rest/deployments/statuses?apiVersion=2022-11-28

# Ideas

- Split into two parts, a manager and worker. There will only be one manager but one worker per

```
gh api --method POST -H "Accept: application/vnd.github+json" -H "X-GitHub-Api-Version: 2022-11-28" /repos/Basicprogrammer10/GithubTests/deployments -f ref='topic-branch' -f payload='{ "deploy": "migrate" }' -f description='Test deploy request. [link?](https://connorcode.com)' 
gh api --method POST -H "Accept: application/vnd.github+json" -H "X-GitHub-Api-Version: 2022-11-28" /repos/Basicprogrammer10/GithubTests/deployments/1035410216/statuses -f environment='production' -f state='success' -f log_url='https://example.com/deployment/42/output' -f description='Deployment finished successfully.' 

- name: Create deployment
      shell: bash
      run: 'gh api --method POST -H "Accept: application/vnd.github+json" -H "X-GitHub-Api-Version: 2022-11-28" /repos/${{ github.repository }}/deployments -f ref=''topic-branch'' -f payload=''{ "deploy": "migrate" }'''
      env:
        GH_TOKEN: ${{ github.token }}

# Send POST to ${{ inputs.ampersand_server }}/api/${{ inputs.ampersand_project }}/deployments
# With header "Authorization: Bearer ${{ inputs.ampersand_token }}"
```