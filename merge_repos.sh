#!/usr/bin/env bash
#
# Script to add a child repository as a subtree into a specified folder
# within a parent repository without squashing history.
#
# Usage:
#   ./merge_subtree.sh -P <parent_repo_path> -C <child_repo_url> -p <subfolder> [-b <branch>] [-R <child_remote_name>]
#
#   -P  Path to the parent repository (destination repository) where the merge will occur.
#       Defaults to the current directory "." if not provided.
#   -C  URL of the child repository to merge (required)
#   -p  Subfolder path in the parent repository where the child repository's files will be placed (required)
#   -b  Branch name from the child repository (default: main)
#   -R  Remote name to assign for the child repository (default: child_remote)
#
usage() {
    echo "Usage: $0 -P <parent_repo_path> -C <child_repo_url> -p <subfolder> [-b <branch>] [-R <child_remote_name>]"
    exit 1
}

# Default values
branch="main"
child_remote_name="child_remote"
parent_repo_path="."

# Parse command-line options
while getopts "P:C:p:b:R:h" opt; do
    case $opt in
    P)
        parent_repo_path="$OPTARG"
        ;;
    C)
        child_repo_url="$OPTARG"
        ;;
    p)
        subfolder="$OPTARG"
        ;;
    b)
        branch="$OPTARG"
        ;;
    R)
        child_remote_name="$OPTARG"
        ;;
    h)
        usage
        ;;
    *)
        usage
        ;;
    esac
done

# Validate required options: child_repo_url and subfolder are mandatory.
if [ -z "$child_repo_url" ] || [ -z "$subfolder" ]; then
    echo "Error: child_repo_url (-C) and subfolder (-p) are required."
    usage
fi

# Check if parent repository path exists and is a directory
if [ ! -d "$parent_repo_path" ]; then
    echo "Error: The parent repository path '$parent_repo_path' does not exist or is not a directory."
    exit 1
fi

# Change to the parent repository directory
cd "$parent_repo_path" || {
    echo "Error: Failed to change directory to '$parent_repo_path'"
    exit 1
}

# Ensure the target directory is a Git repository
if [ ! -d .git ]; then
    echo "Error: The parent repository path '$parent_repo_path' is not a Git repository."
    exit 1
fi

# Add the child repository as a remote if it doesn't already exist
if git remote | grep -q "^${child_remote_name}$"; then
    echo "Remote '${child_remote_name}' already exists. Skipping remote add."
else
    echo "Adding remote '${child_remote_name}' with URL '${child_repo_url}'..."
    git remote add ${child_remote_name} ${child_repo_url}
fi

# Fetch the child repository
echo "Fetching remote '${child_remote_name}'..."
git fetch ${child_remote_name}

# Add the subtree from the child repository without squashing history
echo "Adding subtree from remote '${child_remote_name}' branch '${branch}' into folder '${subfolder}'..."
git subtree add --prefix=${subfolder} ${child_remote_name} ${branch}

echo "Subtree merge complete in parent repository at '$parent_repo_path'."
