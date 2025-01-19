#include <ntddk.h>

NTSTATUS DriverEntry(PDRIVER_OBJECT DriverObject, PUNICODE_STRING RegistryPath) {
    UNREFERENCED_PARAMETER(RegistryPath);

    // Set up driver unloading function
    DriverObject->DriverUnload = UnloadDriver;

    DbgPrint("Rootkit loaded successfully!\n");

    // This driver does nothing yet, but it has loaded successfully.
    return STATUS_SUCCESS;
}

VOID UnloadDriver(PDRIVER_OBJECT DriverObject) {
    UNREFERENCED_PARAMETER(DriverObject);
    DbgPrint("Rootkit unloaded successfully!\n");
}
