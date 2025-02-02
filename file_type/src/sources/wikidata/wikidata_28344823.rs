use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344823: FileFormat = FileFormat {
    id: 28_344_823,
    source_type: SourceType::Wikidata,
    name: "Debian source control file",
    extensions: &["dsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
