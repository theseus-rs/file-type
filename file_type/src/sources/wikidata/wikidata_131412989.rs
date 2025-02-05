use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131412989: FileFormat = FileFormat {
    id: 131_412_989,
    source_type: SourceType::Wikidata,
    name: "VisualProlog file format",
    extensions: &["cl", "i", "pack", "ph", "pro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
