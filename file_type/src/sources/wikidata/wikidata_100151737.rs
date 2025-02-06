use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100151737: FileFormat = FileFormat {
    id: 100_151_737,
    source_type: SourceType::Wikidata,
    name: "Muvee autoProducer Project File",
    extensions: &["mve"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
