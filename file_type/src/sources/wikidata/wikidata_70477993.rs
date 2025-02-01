use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_70477993: FileFormat = FileFormat {
    id: 70_477_993,
    puid: "wikidata/70477993",
    name: "LabSpec spectrometer",
    extensions: &["spc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
