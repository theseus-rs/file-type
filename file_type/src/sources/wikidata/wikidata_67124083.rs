use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67124083: FileFormat = FileFormat {
    id: 67_124_083,
    source_type: SourceType::Wikidata,
    name: "Print Artist label file format",
    extensions: &["lbl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
