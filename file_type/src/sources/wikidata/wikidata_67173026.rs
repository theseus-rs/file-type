use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67173026: FileFormat = FileFormat {
    id: 67_173_026,
    source_type: SourceType::Wikidata,
    name: "GIMP compressed XJT Image",
    extensions: &["xjt", "xjtbz", "xjtgz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
