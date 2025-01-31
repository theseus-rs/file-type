use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2371344: FileFormat = FileFormat {
    id: 2_371_344,
    puid: "wikidata/2371344",
    name: "TeX font metric",
    extensions: &["tfm"],
    media_types: &["application/x-tex-tfm"],
    internal_signatures: &[],
    related_formats: &[],
};
