use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131860033: FileFormat = FileFormat {
    id: 131_860_033,
    source_type: SourceType::Wikidata,
    name: "VPIC file format",
    extensions: &["vpc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
