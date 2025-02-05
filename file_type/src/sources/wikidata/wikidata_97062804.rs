use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_97062804: FileFormat = FileFormat {
    id: 97_062_804,
    source_type: SourceType::Wikidata,
    name: "X-Motif UIL table",
    extensions: &["uil"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
