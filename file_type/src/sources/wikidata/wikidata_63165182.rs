use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63165182: FileFormat = FileFormat {
    id: 63_165_182,
    puid: "wikidata/63165182",
    name: "Microsoft Office Binder Wizard for Windows",
    extensions: &["obz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
