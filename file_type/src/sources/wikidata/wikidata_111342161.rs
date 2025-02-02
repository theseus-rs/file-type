use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342161: FileFormat = FileFormat {
    id: 111_342_161,
    source_type: SourceType::Wikidata,
    name: "Ad Lib Gold sample",
    extensions: &["smp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
