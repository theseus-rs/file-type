use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086768: FileFormat = FileFormat {
    id: 110_086_768,
    source_type: SourceType::Wikidata,
    name: "Agisoft Project Archive",
    extensions: &["psz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
