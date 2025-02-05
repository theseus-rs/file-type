use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2371344: FileFormat = FileFormat {
    id: 2_371_344,
    source_type: SourceType::Wikidata,
    name: "TeX font metric",
    extensions: &["tfm"],
    media_types: &["application/x-tex-tfm"],
    signatures: &[],
    related_formats: &[],
};
