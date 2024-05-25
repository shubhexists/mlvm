remove_mvm_from_path() {
    local new_path=""
    local first=true
    for dir in $(echo $PATH | tr ":" "\n"); do
        if [[ "$dir" != *".mvm"* ]]; then
            if [ "$first" = true ]; then
                new_path="$dir"
                first=false
            else
                new_path="$new_path:$dir"
            fi
        fi
    done
    export PATH="$new_path"
}

add_mvm_to_path() {
    local path=$1
    if [[ "$PATH" != *"$path"* ]]; then
        export PATH="$path:$PATH"
    fi
}