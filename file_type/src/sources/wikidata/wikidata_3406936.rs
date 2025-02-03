use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3406936: FileFormat = FileFormat {
    id: 3_406_936,
    source_type: SourceType::Wikidata,
    name: "Program database",
    extensions: &["pdb"],
    media_types: &["application/x-ms-pdb"],
    internal_signatures: &[],
    related_formats: &[],
};
