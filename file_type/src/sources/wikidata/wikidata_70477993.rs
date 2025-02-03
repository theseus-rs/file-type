use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_70477993: FileFormat = FileFormat {
    id: 70_477_993,
    source_type: SourceType::Wikidata,
    name: "LabSpec spectrometer",
    extensions: &["spc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
