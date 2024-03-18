" devicons
set encoding=UTF-8
let g:airline_powerline_fonts = 1

" vertical rulers
set colorcolumn=72,80,120,240
highlight ColorColumn term=reverse cterm=NONE ctermfg=NONE gui=NONE guifg=NONE guibg=blue ctermbg=6

set nocompatible              " required
filetype off                  " required

filetype plugin indent on    " required

set number                     " Show current line number
set relativenumber             " Show relative line numbers


" enable syntax highlighting
syntax enable

" dark background
set background=dark

" solarized
" colorscheme solarized

" show line numbers
set number

" set tabs to be 4 spaces
set ts=4

" indent when writing code
set autoindent

" Load tagging
set tags=./tags;,tags;

" Rust
let g:rustfmt_autosave = 1
" let g:deoplete#enable_at_startup = 1
let g:neosnippet#enable_complete_done = 1
let g:LanguageClient_serverCommands = {
    \ 'rust': ['~/.cargo/bin/ra_lsp_server'],
    \ }

" Python
let python_highlight_all = 1
let g:vim_isort_python_version = 'python3'
let g:rainbow_active = 1

" Run ruff format on save
autocmd BufWritePost *.py execute ':RuffFmt'

" Automatic installation of vim-plug
if empty(glob('~/.vim/autoload/plug.vim'))
  silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
    \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
  autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

" Automatic update of language server
" Plug 'autozimu/LanguageClient-neovim', {
"     \ 'branch': 'next',
"     \ 'do': 'bash install.sh',
"     \ }

" let g:LanguageClient_serverCommands = {
"     \ 'rust': ['~/.cargo/bin/rustup', 'run', 'stable', 'rls'],
"     \ 'python': ['/usr/local/bin/pyls'],
"     \ }

" Automatic update of ctags
function! DelTagOfFile(file)
  let fullpath = a:file
  let cwd = getcwd()
  let tagfilename = cwd . "/tags"
  let f = substitute(fullpath, cwd . "/", "", "")
  let f = escape(f, './')
  let cmd = 'sed -i "/' . f . '/d" "' . tagfilename . '"'
  let resp = system(cmd)
endfunction

function! UpdateTags()
  let f = expand("%:p")
  let cwd = getcwd()
  let tagfilename = cwd . "/tags"
  let cmd = 'ctags -a -f ' . tagfilename . ' --c++-kinds=+p --fields=+iaS --extra=+q ' . '"' . f . '"'
  call DelTagOfFile(f)
  let resp = system(cmd)
endfunction
autocmd BufWritePost *.cpp,*.h,*.c call UpdateTags()


" Specify a directory for plugins
" - For Neovim: ~/.local/share/nvim/plugged
" - Avoid using standard Vim directory names like 'plugin'
call plug#begin('~/.vim/plugged')

" Copilot
Plug 'github/copilot.vim'

" Intellisense
Plug 'neoclide/coc.nvim', {'tag': '*', 'do': { -> coc#util#install()}}
" Plug 'neoclide/coc.nvim', {'branch': 'release'}
" Plug 'neoclide/coc.nvim', {'do': { -> coc#util#install()}}

Plug 'ervandew/supertab'
Plug 'junegunn/fzf', { 'dir': '~/.fzf', 'do': './install --all' }

Plug 'lifepillar/vim-solarized8'
Plug 'flazz/vim-colorschemes'
Plug 'sheerun/vim-polyglot'
Plug 'severin-lemaignan/vim-minimap'
Plug 'frazrepo/vim-rainbow'

Plug 'mhinz/vim-signify'
Plug 'scrooloose/nerdtree'
Plug 'airblade/vim-gitgutter'
Plug 'tiagofumo/vim-nerdtree-syntax-highlight'

" Color theme
Plug 'micha/vim-colors-solarized'


" Rust
Plug 'rust-lang/rust.vim',    { 'for': 'rust' }
Plug 'autozimu/LanguageClient-neovim', {
    \ 'branch': 'next',
    \ 'do': 'bash install.sh',
    \ }

Plug 'roxma/nvim-yarp'
Plug 'roxma/vim-hug-neovim-rpc'

" if has('nvim')
"   Plug 'Shougo/deoplete.nvim', { 'do': ':UpdateRemotePlugins' }
" else
"   # Plug 'Shougo/deoplete.nvim'
"   Plug 'roxma/nvim-yarp'
"   Plug 'roxma/vim-hug-neovim-rpc'
" endif

Plug 'Shougo/neosnippet.vim'
Plug 'Shougo/neosnippet-snippets'


" Python
Plug 'flebel/vim-mypy', { 'for': 'python', 'branch': 'bugfix/fast_parser_is_default_and_only_parser' }

" Syntax highlighting system/plugin
Plug 'scrooloose/syntastic'

" Must be last
Plug 'ryanoasis/vim-devicons'

" Initialize plugin system
call plug#end()
