use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58725633: FileFormat = FileFormat {
    id: 58_725_633,
    source_type: SourceType::Wikidata,
    name: "Adobe PostScript Font Metrics file",
    extensions: &["pfm"],
    media_types: &["application/x-font-pfm"],
    signatures: &[],
    related_formats: &[],
};
