use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111356275: FileFormat = FileFormat {
    id: 111_356_275,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif ES 'waves' format",
    extensions: &["w7w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
