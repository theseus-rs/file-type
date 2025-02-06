use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63106845: FileFormat = FileFormat {
    id: 63_106_845,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder Template for Windows",
    extensions: &["obt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
