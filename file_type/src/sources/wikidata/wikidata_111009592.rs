use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009592: FileFormat = FileFormat {
    id: 111_009_592,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Label File format",
    extensions: &["lbl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
