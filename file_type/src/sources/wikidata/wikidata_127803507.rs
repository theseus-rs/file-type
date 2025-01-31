use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127803507: FileFormat = FileFormat {
    id: 127_803_507,
    puid: "wikidata/127803507",
    name: "Mojo source code file",
    extensions: &["mojo", "mojo", "🔥", "🔥"],
    media_types: &[
        "application/x-mojo",
        "application/x-mojo",
        "text/x-mojo",
        "text/x-mojo",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
