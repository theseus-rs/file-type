use crate::format::FileFormat;

pub(crate) const LINGUIST_104: FileFormat = FileFormat {
    id: 104,
    puid: "linguist/104",
    name: "Erlang",
    extensions: &[
        "app", "app.src", "erl", "es", "escript", "hrl", "xrl", "yrl",
    ],
    media_types: &["text/x-erlang"],
    internal_signatures: &[],
    related_formats: &[],
};
