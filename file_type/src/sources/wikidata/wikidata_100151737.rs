use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100151737: FileFormat = FileFormat {
    id: 100_151_737,
    source_type: SourceType::Wikidata,
    name: "Muvee autoProducer Project File",
    extensions: &["mve"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
