use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51837664: FileFormat = FileFormat {
    id: 51_837_664,
    source_type: SourceType::Wikidata,
    name: "Micrografx Designer format",
    extensions: &["dsf"],
    media_types: &["application/x-mgx-designer"],
    internal_signatures: &[],
    related_formats: &[],
};
