use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117457195: FileFormat = FileFormat {
    id: 117_457_195,
    puid: "wikidata/117457195",
    name: "Raw PIMA SWIR Reflectance Spectral File",
    extensions: &["fos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
