use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111418374: FileFormat = FileFormat {
    id: 111_418_374,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge Cache File",
    extensions: &["bc", "bcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
