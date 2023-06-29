# Compile and run
``. run.sh`

# Push to personal Gerrit branch
`git push origin HEAD:<redacted>`
- To make changes (patches) to commit not yet published to origin: commit amend
- No "active" commit to patch, create new commit: commit -m 

## Committing Process
- Add changes
- Commit (see above)
- Fetch
- Rebase
- Push as shown above (if you want; see above)
- Go to Gerrit, submit changes (if commit ready to merge)
- Pull (mandatory after submitting!)

### Alternative Committing 
- Stash
- Fetch
- Rebase
- Stash pop
- Add
- Commit
- Push as shown above (if you want; see above)
- Go to Gerrit, submit changes (if commit ready to merge)
- Pull (mandatory after submitting!)
