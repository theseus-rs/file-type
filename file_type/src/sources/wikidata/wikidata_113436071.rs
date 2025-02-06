use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113436071: FileFormat = FileFormat {
    id: 113_436_071,
    source_type: SourceType::Wikidata,
    name: "INTREPID Standard Information File",
    extensions: &["isi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
