use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7271522: FileFormat = FileFormat {
    id: 7_271_522,
    puid: "wikidata/7271522",
    name: "Question Object File Format",
    extensions: &["quiz", "quox"],
    media_types: &[
        "application/vnd.quobject-quoxdocument",
        "application/vnd.quobject-quoxdocument",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
