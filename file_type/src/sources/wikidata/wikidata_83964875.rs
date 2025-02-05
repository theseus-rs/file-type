use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83964875: FileFormat = FileFormat {
    id: 83_964_875,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Workgroup Information File",
    extensions: &["mdw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
