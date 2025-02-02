use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121760718: FileFormat = FileFormat {
    id: 121_760_718,
    source_type: SourceType::Wikidata,
    name: "Adobe Acrobat MIME Encoded Job Definition File",
    extensions: &["mjd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
