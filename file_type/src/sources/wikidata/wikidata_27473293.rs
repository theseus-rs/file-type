use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473293: FileFormat = FileFormat {
    id: 27_473_293,
    source_type: SourceType::Wikidata,
    name: "CADRG Overview Image",
    extensions: &["ovr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
