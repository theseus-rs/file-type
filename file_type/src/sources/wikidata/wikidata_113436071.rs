use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113436071: FileFormat = FileFormat {
    id: 113_436_071,
    source_type: SourceType::Wikidata,
    name: "INTREPID Standard Information File",
    extensions: &["isi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
