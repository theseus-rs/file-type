use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1079778: FileFormat = FileFormat {
    id: 1_079_778,
    source_type: SourceType::Wikidata,
    name: "SIS",
    extensions: &["sis", "sisx"],
    media_types: &["application/vnd.symbian.install"],
    internal_signatures: &[],
    related_formats: &[],
};
