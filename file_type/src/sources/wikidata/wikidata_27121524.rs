use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27121524: FileFormat = FileFormat {
    id: 27_121_524,
    source_type: SourceType::Wikidata,
    name: "fixed width text file",
    extensions: &["fwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
