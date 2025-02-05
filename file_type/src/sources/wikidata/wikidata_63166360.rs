use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63166360: FileFormat = FileFormat {
    id: 63_166_360,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder Wizard for Windows, version 97-2003",
    extensions: &["obz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
