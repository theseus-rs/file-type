use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122021046: FileFormat = FileFormat {
    id: 122_021_046,
    source_type: SourceType::Wikidata,
    name: "Retina file",
    extensions: &["rtd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
