use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1079778: FileFormat = FileFormat {
    id: 1_079_778,
    source_type: SourceType::Wikidata,
    name: "SIS",
    extensions: &["sis", "sisx"],
    media_types: &["application/vnd.symbian.install"],
    signatures: &[],
    related_formats: &[],
};
