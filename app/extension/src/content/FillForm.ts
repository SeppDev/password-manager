export function FillAccount(email: string | undefined, username: string | undefined, password: string | undefined) {
    handleQuery("input[type='email']", email);
    handleQuery("input[type='text']", username);
    handleQuery("input[type='password']", password);
}

function handleQuery(query: string, value: string | undefined) {
    if (!value) return;

    document
        .querySelectorAll(query)
        .forEach((element) => SetValue(element as HTMLInputElement, value));
}

export function Submit() {
    const buttons: HTMLElement[] = [];

    function pushElements(elements: NodeListOf<HTMLElement>) {
        elements.forEach((element) => buttons.push(element))
    }

    pushElements(document.querySelectorAll("button[id='submit']"));
    pushElements(document.querySelectorAll("input[type='submit']"));
    pushElements(document.querySelectorAll("button[type='submit']"));

    const first: HTMLElement | undefined = buttons[0];

    if (!first) return;
    Click(first);
}

function Click(element: HTMLElement) {
    const submit = element as HTMLInputElement | HTMLButtonElement;
    submit.click();
}

function SetValue(input: HTMLInputElement, value: string) {
    input.value = value;
}
