use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086818: FileFormat = FileFormat {
    id: 110_086_818,
    source_type: SourceType::Wikidata,
    name: "Agisoft Project File",
    extensions: &["psx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
