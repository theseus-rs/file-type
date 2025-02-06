use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_70477993: FileFormat = FileFormat {
    id: 70_477_993,
    source_type: SourceType::Wikidata,
    name: "LabSpec spectrometer",
    extensions: &["spc"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
