use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205982: FileFormat = FileFormat {
    id: 28_205_982,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Q Color Channel",
    extensions: &["imq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
